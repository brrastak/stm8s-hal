#[doc = "Register `AWCRH` reader"]
pub type R = crate::R<AWCRH_SPEC>;
#[doc = "Register `AWCRH` writer"]
pub type W = crate::W<AWCRH_SPEC>;
#[doc = "Field `AWEN8` reader - "]
pub type AWEN8_R = crate::BitReader;
#[doc = "Field `AWEN8` writer - "]
pub type AWEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWEN9` reader - "]
pub type AWEN9_R = crate::BitReader;
#[doc = "Field `AWEN9` writer - "]
pub type AWEN9_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn awen8(&self) -> AWEN8_R {
        AWEN8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn awen9(&self) -> AWEN9_R {
        AWEN9_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn awen8(&mut self) -> AWEN8_W<AWCRH_SPEC> {
        AWEN8_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn awen9(&mut self) -> AWEN9_W<AWCRH_SPEC> {
        AWEN9_W::new(self, 1)
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
#[doc = "ADC analog watchdog control register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awcrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awcrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWCRH_SPEC;
impl crate::RegisterSpec for AWCRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`awcrh::R`](R) reader structure"]
impl crate::Readable for AWCRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awcrh::W`](W) writer structure"]
impl crate::Writable for AWCRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWCRH to value 0"]
impl crate::Resettable for AWCRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
