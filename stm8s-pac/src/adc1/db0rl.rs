#[doc = "Register `DB0RL` reader"]
pub type R = crate::R<DB0RL_SPEC>;
#[doc = "Register `DB0RL` writer"]
pub type W = crate::W<DB0RL_SPEC>;
#[doc = "Field `DL` reader - "]
pub type DL_R = crate::FieldReader;
#[doc = "Field `DL` writer - "]
pub type DL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn dl(&mut self) -> DL_W<DB0RL_SPEC> {
        DL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC data buffer registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db0rl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db0rl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DB0RL_SPEC;
impl crate::RegisterSpec for DB0RL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`db0rl::R`](R) reader structure"]
impl crate::Readable for DB0RL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`db0rl::W`](W) writer structure"]
impl crate::Writable for DB0RL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DB0RL to value 0"]
impl crate::Resettable for DB0RL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
