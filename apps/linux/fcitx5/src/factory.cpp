//! 易言 addon 工厂。
#include "yiyan.h"
namespace fcitx {
class YiyanFactory final : public AddonFactory {
public:
    AddonInstance *create(AddonManager *manager) override { return new YiyanEngine(manager); }
};
}
FCITX_ADDON_FACTORY_V2(yiyan, fcitx::YiyanFactory)
