#[doc = "Register `CCER2` reader"]
pub type R = crate::R<CCER2_SPEC>;
#[doc = "Register `CCER2` writer"]
pub type W = crate::W<CCER2_SPEC>;
#[doc = "Field `CC3E` reader - "]
pub type CC3E_R = crate::BitReader;
#[doc = "Field `CC3E` writer - "]
pub type CC3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3P` reader - "]
pub type CC3P_R = crate::BitReader;
#[doc = "Field `CC3P` writer - "]
pub type CC3P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NE` reader - "]
pub type CC3NE_R = crate::BitReader;
#[doc = "Field `CC3NE` writer - "]
pub type CC3NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NP` reader - "]
pub type CC3NP_R = crate::BitReader;
#[doc = "Field `CC3NP` writer - "]
pub type CC3NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4E` reader - "]
pub type CC4E_R = crate::BitReader;
#[doc = "Field `CC4E` writer - "]
pub type CC4E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4P` reader - "]
pub type CC4P_R = crate::BitReader;
#[doc = "Field `CC4P` writer - "]
pub type CC4P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cc3ne(&self) -> CC3NE_R {
        CC3NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cc3e(&mut self) -> CC3E_W<CCER2_SPEC> {
        CC3E_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cc3p(&mut self) -> CC3P_W<CCER2_SPEC> {
        CC3P_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cc3ne(&mut self) -> CC3NE_W<CCER2_SPEC> {
        CC3NE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cc3np(&mut self) -> CC3NP_W<CCER2_SPEC> {
        CC3NP_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cc4e(&mut self) -> CC4E_W<CCER2_SPEC> {
        CC4E_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cc4p(&mut self) -> CC4P_W<CCER2_SPEC> {
        CC4P_W::new(self, 5)
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
#[doc = "TIM1 capture/compare enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCER2_SPEC;
impl crate::RegisterSpec for CCER2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccer2::R`](R) reader structure"]
impl crate::Readable for CCER2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccer2::W`](W) writer structure"]
impl crate::Writable for CCER2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCER2 to value 0"]
impl crate::Resettable for CCER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
