local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({
    "git",
    "clone",
    "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git",
    "--branch=stable", -- latest stable release
    lazypath,
  })
end
vim.opt.rtp:prepend(lazypath)
require("lazy").setup({
 	{
   		"folke/tokyonight.nvim",
  		lazy = false,
  		priority = 1000,
  		opts = {},
 	}
})
require("tokyonight").setup({
	style = "night",
 	on_colors = function(colors)
		colors.bg = "#000000"
		colors.bg_dark= "#000000"
 	end
})
vim.cmd([[set bg=dark]])
vim.cmd([[colorscheme tokyonight]])
vim.cmd([[set ts=4 sw=4 et]])
vim.cmd([[filetype plugin on]])
