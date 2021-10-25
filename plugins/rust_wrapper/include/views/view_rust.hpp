#pragma once

#include <hex.hpp>

#include <hex/views/view.hpp>
#include <hex/helpers/logger.hpp>

#include <imgui.h>

#include <functional>
#include <string>
#include <vector>

namespace hex::rust {

    using ViewRustSimpleFunc = ::rust::Fn<void()>;
    using ViewRustBoolFunc = ::rust::Fn<bool()>;
    using ViewRustContentDrawFunc = ::rust::Fn<void(bool&)>;

    struct ViewRustFunctions {
        bool destructorValid;
        ViewRustSimpleFunc destructorFunc;

        bool drawContentValid;
        ViewRustContentDrawFunc drawContentFunc;

        bool drawAlwaysVisibleValid;
        ViewRustSimpleFunc drawAlwaysVisibleFunc;

        bool drawMenuValid;
        ViewRustSimpleFunc drawMenuFunc;

        bool isAvailableValid;
        ViewRustBoolFunc isAvailableFunc;

        bool shouldProcessValid;
        ViewRustBoolFunc shouldProcessFunc;

        bool hasViewMenuItemEntryValid;
        ViewRustBoolFunc hasViewMenuItemEntryFunc;
    };

    class ViewRustWrapper : public View {
    public:
        explicit ViewRustWrapper(std::string unlocalizedViewName, ViewRustFunctions funcs) : View(unlocalizedViewName), m_funcs(funcs) {

        }
        ~ViewRustWrapper() override {
            if (this->m_funcs.destructorValid)
                this->m_funcs.destructorFunc();
            else {
                // View::~View(); Already called by destructor anyways
            }
        }

        void drawContent() override {
            if (this->m_funcs.drawContentValid)
                this->m_funcs.drawContentFunc(this->getWindowOpenState());
            else {
                hex::log::fatal("Rust View 'void drawContent()' was not implemented!");
                std::abort();
            }
        }

        void drawAlwaysVisible() override {
            if (this->m_funcs.drawAlwaysVisibleValid)
                this->m_funcs.drawAlwaysVisibleFunc();
            else
                View::drawAlwaysVisible();
        }

        void drawMenu() override {
            if (this->m_funcs.drawMenuValid)
                this->m_funcs.drawMenuFunc();
            else
                View::drawMenu();
        }

        bool handleShortcut(bool keys[512], bool ctrl, bool shift, bool alt) override {
            return View::handleShortcut(keys, ctrl, shift, alt);
        }

        bool isAvailable() override {
            if (this->m_funcs.isAvailableValid)
                return this->m_funcs.isAvailableFunc();
            else
                return View::isAvailable();
        }

        bool shouldProcess() override {
            if (this->m_funcs.shouldProcessValid)
                return this->m_funcs.shouldProcessFunc();
            else
                return View::shouldProcess();
        }

        bool hasViewMenuItemEntry() override {
            if (this->m_funcs.hasViewMenuItemEntryValid)
                return this->m_funcs.hasViewMenuItemEntryFunc();
            else
                return View::hasViewMenuItemEntry();
        }

        ImVec2 getMinSize() override {
            return View::getMinSize();
        }

        ImVec2 getMaxSize() override {
            return View::getMaxSize();
        }

    private:
        ViewRustFunctions m_funcs;
    };

}