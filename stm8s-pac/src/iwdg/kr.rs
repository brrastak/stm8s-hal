#[doc = "Register `KR` reader"]
pub type R = crate::R<KR_SPEC>;
#[doc = "Register `KR` writer"]
pub type W = crate::W<KR_SPEC>;
#[doc = "Field `KEY` reader - "]
pub type KEY_R = crate::FieldReader;
#[doc = "Field `KEY` writer - "]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KR_SPEC> {
        KEY_W::new(self, 0)
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
#[doc = "IWDG key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`kr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`kr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KR_SPEC;
impl crate::RegisterSpec for KR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kr::R`](R) reader structure"]
impl crate::Readable for KR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`kr::W`](W) writer structure"]
impl crate::Writable for KR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KR to value 0"]
impl crate::Resettable for KR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
