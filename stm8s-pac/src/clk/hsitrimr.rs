#[doc = "Register `HSITRIMR` reader"]
pub type R = crate::R<HSITRIMR_SPEC>;
#[doc = "Register `HSITRIMR` writer"]
pub type W = crate::W<HSITRIMR_SPEC>;
#[doc = "Field `HSITRIM` reader - "]
pub type HSITRIM_R = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - "]
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<HSITRIMR_SPEC> {
        HSITRIM_W::new(self, 0)
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
#[doc = "HSI clock calibration trimming register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsitrimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsitrimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSITRIMR_SPEC;
impl crate::RegisterSpec for HSITRIMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hsitrimr::R`](R) reader structure"]
impl crate::Readable for HSITRIMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsitrimr::W`](W) writer structure"]
impl crate::Writable for HSITRIMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSITRIMR to value 0"]
impl crate::Resettable for HSITRIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
