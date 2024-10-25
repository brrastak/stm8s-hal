#[doc = "Register `APR` reader"]
pub type R = crate::R<APR_SPEC>;
#[doc = "Register `APR` writer"]
pub type W = crate::W<APR_SPEC>;
#[doc = "Field `APR` reader - "]
pub type APR_R = crate::FieldReader;
#[doc = "Field `APR` writer - "]
pub type APR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn apr(&self) -> APR_R {
        APR_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn apr(&mut self) -> APR_W<APR_SPEC> {
        APR_W::new(self, 0)
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
#[doc = "AWU asynchronous prescaler buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APR_SPEC;
impl crate::RegisterSpec for APR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`apr::R`](R) reader structure"]
impl crate::Readable for APR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apr::W`](W) writer structure"]
impl crate::Writable for APR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APR to value 0x3f"]
impl crate::Resettable for APR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
