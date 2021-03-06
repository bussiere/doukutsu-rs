use num_traits::clamp;

use crate::common::Direction;
use crate::ggez::GameResult;
use crate::npc::NPC;
use crate::player::Player;
use crate::SharedGameState;

impl NPC {
    pub(crate) fn tick_n071_chinfish(&mut self, state: &mut SharedGameState) -> GameResult {
        if self.action_num == 0 {
            self.action_num = 1;
            self.target_x = self.x;
            self.target_y = self.y;
            self.vel_y = 0x80;
        }

        if self.action_num == 1 {
            if self.target_y < self.y {
                self.vel_y -= 8;
            } else if self.target_y > self.y {
                self.vel_y += 8;
            }

            self.vel_y = clamp(self.vel_y, -0x100, 0x100);
        }

        self.x += self.vel_x;
        self.y += self.vel_y;

        self.anim_counter += 1;
        if self.anim_counter > 4 {
            self.anim_counter = 0;
            self.anim_num += 1;
        }

        if self.anim_num > 1 {
            self.anim_num = 0;
        }

        if self.shock > 0 {
            self.anim_num = 2;
        }

        if self.direction == Direction::Left {
            self.anim_rect = state.constants.npc.n071_chinfish[self.anim_num as usize];
        } else {
            self.anim_rect = state.constants.npc.n071_chinfish[self.anim_num as usize + 3];
        }

        Ok(())
    }

    pub(crate) fn tick_n075_kanpachi(&mut self, state: &mut SharedGameState, player: &Player) -> GameResult {
        if self.action_num == 0 {
            self.action_num = 1;
            self.anim_num = 0;
            self.anim_counter = 0;
        }

        if self.action_num == 1 {
            if (self.x - (48 * 0x200) < player.x) && (self.x + (48 * 0x200) > player.x)
                && (self.y - (48 * 0x200) < player.y) && (self.y + (48 * 0x200) > player.y) {
                self.anim_num = 1;
            } else {
                self.anim_num = 0;
            }
        }

        self.anim_rect = state.constants.npc.n075_kanpachi[self.anim_num as usize];

        Ok(())
    }

    pub(crate) fn tick_n077_yamashita(&mut self, state: &mut SharedGameState) -> GameResult {
        if self.action_num == 0 {
            self.action_num = 1;
            self.anim_num = 0;
            self.anim_counter = 0;
        }

        match self.action_num {
            1 => {
                if state.game_rng.range(0..120) == 10 {
                    self.action_num = 2;
                    self.action_counter = 0;
                    self.anim_num = 1;
                }
            }
            2 => {
                self.action_counter += 1;
                if self.action_counter > 8 {
                    self.action_num = 1;
                    self.anim_num = 0;
                }
            }
            _ => {}
        }

        if self.direction == Direction::Left {
            self.anim_rect = state.constants.npc.n077_yamashita[self.anim_num as usize];
        } else {
            self.anim_rect = state.constants.npc.n077_yamashita[2];
        }

        Ok(())
    }


    pub(crate) fn tick_n079_mahin(&mut self, state: &mut SharedGameState, player: &Player) -> GameResult {
        match self.action_num {
            0 => {
                self.action_num = 1;
                self.anim_num = 2;
                self.anim_counter = 0;
            }
            2 => {
                self.anim_num = 0;
                if state.game_rng.range(0..120) == 10 {
                    self.action_num = 3;
                    self.action_counter = 0;
                    self.anim_num = 1;
                }

                if (self.x - (32 * 0x200) < player.x) && (self.x + (32 * 0x200) > player.x)
                    && (self.y - (32 * 0x200) < player.y) && (self.y + (16 * 0x200) > player.y) {

                    if self.x > player.x {
                        self.direction = Direction::Left;
                    } else {
                        self.direction = Direction::Right;
                    }
                }
            }
            3 => {
                self.action_counter += 1;
                if self.action_counter > 8 {
                    self.action_num = 2;
                    self.anim_num = 0;
                }
            }
            _ => {}
        }


        self.vel_y += 0x40;

        if self.vel_y > 0x5ff {
            self.vel_y = 0x5ff;
        }

        self.y += self.vel_y;

        if self.direction == Direction::Left {
            self.anim_rect = state.constants.npc.n079_mahin[self.anim_num as usize];
        } else {
            self.anim_rect = state.constants.npc.n079_mahin[self.anim_num as usize + 3];
        }

        Ok(())
    }
}
