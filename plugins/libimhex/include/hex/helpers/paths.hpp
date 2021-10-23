#pragma once

#include <functional>
#include <string>
#include <vector>

#include <nfd.hpp>

namespace hex {

    enum class ImHexPath {
        Patterns,
        PatternsInclude,
        Magic,
        Python,
        Plugins,
        Yara,
        Config,
        Resources,
        Constants
    };

    std::vector<std::string> getPath(ImHexPath path);

    enum class DialogMode {
        Open,
        Save,
        Folder
    };

    void openFileBrowser(const std::string &title, DialogMode mode, const std::vector<nfdfilteritem_t> &validExtensions, const std::function<void(std::string)> &callback, const std::string &defaultPath = {});

}