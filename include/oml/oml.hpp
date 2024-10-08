#pragma once

#include <cstdint>
#include <format>
#include <memory>
#include <string>
#include <string_view>
#include <variant>
#include <vector>

extern "C" {
#include "oml.h"
}

namespace oml {
class OmlExpr;
class OmlValue {
  friend class OmlExpr;

  OmlValue(void *pval, std::string path = "") : pval_(std::shared_ptr<void>(pval, oml_release_value)), path_(path) {}
  OmlValue(std::shared_ptr<void> pval, std::string path = "") : pval_(pval), path_(path) {}

 public:
  bool is_none() { return !!oml_value_is_none(pval_.get(), path_.c_str()); }
  bool is_bool() { return !!oml_value_is_bool(pval_.get(), path_.c_str()); }
  bool is_int() { return !!oml_value_is_int(pval_.get(), path_.c_str()); }
  bool is_float() { return !!oml_value_is_float(pval_.get(), path_.c_str()); }
  bool is_str() { return !!oml_value_is_str(pval_.get(), path_.c_str()); }
  bool is_array() { return !!oml_value_is_array(pval_.get(), path_.c_str()); }
  bool is_map() { return !!oml_value_is_map(pval_.get(), path_.c_str()); }

  void set_none() { oml_value_set_none(pval_.get(), path_.c_str()); }
  void set_bool(bool val) { oml_value_set_bool(pval_.get(), path_.c_str(), val ? 1 : 0); }
  void set_int(int64_t val) { oml_value_set_int(pval_.get(), path_.c_str(), val); }
  void set_float(double val) { oml_value_set_float(pval_.get(), path_.c_str(), val); }
  void set_string(std::string val) { oml_value_set_string(pval_.get(), path_.c_str(), val.c_str()); }

  bool as_bool() { return !!oml_value_as_bool(pval_.get(), path_.c_str()); }
  int64_t as_int() { return oml_value_as_int(pval_.get(), path_.c_str()); }
  double as_float() { return oml_value_as_float(pval_.get(), path_.c_str()); }
  std::string as_str() {
    auto str = oml_value_as_str(pval_.get(), path_.c_str());
    std::string ret = str;
    oml_release_str(str);
    return ret;
  }
  int32_t get_array_length() { return oml_value_get_array_length(pval_.get(), path_.c_str()); }
  int32_t get_map_length() { return oml_value_get_map_length(pval_.get(), path_.c_str()); }
  std::vector<std::string> get_map_keys() {
    auto str = oml_value_get_keys(pval_.get(), path_.c_str());
    std::string_view strv = str;
    std::vector<std::string> ret;
    while (size_t p = strv.find("#")) {
      ret.push_back(std::string(strv.substr(0, p)));
      strv = strv.substr(p + 1);
    }
    if (!strv.empty()) ret.push_back(std::string(strv));
    oml_release_str(str);
    return ret;
  }

  OmlValue operator[](std::string path) {
    if (path_ != "") {
      path = std::format("{}.{}", path_, path);
    }
    return OmlValue(pval_, path);
  }

 private:
  std::shared_ptr<void> pval_;
  std::string path_;
};

class OmlExpr {
  OmlExpr(void *pexpr, std::string path = "") : pexpr_(std::shared_ptr<void>(pexpr, oml_release_expr)), path_(path) {}
  OmlExpr(std::shared_ptr<void> pexpr, std::string path = "") : pexpr_(pexpr), path_(path) {}

 public:
  void set_none() { oml_expr_set_none(pexpr_.get(), path_.c_str()); }
  void set_bool(bool val) { oml_expr_set_bool(pexpr_.get(), path_.c_str(), val ? 1 : 0); }
  void set_int(int64_t val) { oml_expr_set_int(pexpr_.get(), path_.c_str(), val); }
  void set_float(double val) { oml_expr_set_float(pexpr_.get(), path_.c_str(), val); }
  void set_string(std::string val) { oml_expr_set_string(pexpr_.get(), path_.c_str(), val.c_str()); }

  inline static std::variant<OmlExpr, std::string> from_str(const std::string &str) {
    void *pexpr = nullptr;
    const char *perr = nullptr;
    if (!!oml_expr_from_str(str.c_str(), &pexpr, &perr)) {
      return OmlExpr(pexpr);
    } else {
      std::string err = perr;
      oml_release_str(perr);
      return err;
    }
  }

  std::variant<OmlValue, std::string> evalute() {
    void *pval = nullptr;
    const char *perr = nullptr;
    if (!!oml_expr_evalute(pexpr_.get(), path_.c_str(), &pval, &perr)) {
      return OmlValue(pval);
    } else {
      std::string err = perr;
      oml_release_str(perr);
      return err;
    }
  }

  OmlValue operator[](std::string path) {
    if (path_ != "") {
      path = std::format("{}.{}", path_, path);
    }
    return OmlValue(pexpr_, path);
  }

 private:
  std::shared_ptr<void> pexpr_;
  std::string path_;
};
}  // namespace oml
