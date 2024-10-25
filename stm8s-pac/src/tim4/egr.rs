#[doc = "Register `EGR` reader"]
pub type R = crate::R<EGR_SPEC>;
#[doc = "Register `EGR` writer"]
pub type W = crate::W<EGR_SPEC>;
#[doc = "Field `UG` reader - "]
pub type UG_R = crate::BitReader;
#[doc = "Field `UG` writer - "]
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` reader - "]
pub type TG_R = crate::BitReader;
#[doc = "Field `TG` writer - "]
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ug(&self) -> UG_R {
        UG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tg(&self) -> TG_R {
        TG_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<EGR_SPEC> {
        UG_W::new(self, 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<EGR_SPEC> {
        TG_W::new(self, 6)
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
#[doc = "TIM4 event generation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`egr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`egr::R`](R) reader structure"]
impl crate::Readable for EGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
