#! /usr/bin/sh

# Copyright (C) 2024 Lily Lyons
# 
# This file is part of rsgss.
# 
# rsgss is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
# 
# rsgss is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License
# along with rsgss.  If not, see <http://www.gnu.org/licenses/>.

# cd to location of bash script
cd $(dirname "$0")

# download or update ruby-build
if [ -d "ruby-build" ]; then
  # silence!!!!
  git -C ruby-build pull > /dev/null
else
  git clone https://github.com/rbenv/ruby-build.git
fi

ruby-build/bin/ruby-build "3.1.4" pfx/