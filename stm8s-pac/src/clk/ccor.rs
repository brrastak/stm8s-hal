#[doc = "Register `CCOR` reader"]
pub type R = crate::R<CCOR_SPEC>;
#[doc = "Register `CCOR` writer"]
pub type W = crate::W<CCOR_SPEC>;
#[doc = "Field `CCOEN` reader - "]
pub type CCOEN_R = crate::BitReader;
#[doc = "Field `CCOEN` writer - "]
pub type CCOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCOSEL` reader - "]
pub type CCOSEL_R = crate::FieldReader;
#[doc = "Field `CCOSEL` writer - "]
pub type CCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CCORDY` reader - "]
pub type CCORDY_R = crate::BitReader;
#[doc = "Field `CCORDY` writer - "]
pub type CCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0BSY` reader - "]
pub type CC0BSY_R = crate::BitReader;
#[doc = "Field `CC0BSY` writer - "]
pub type CC0BSY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ccoen(&self) -> CCOEN_R {
        CCOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn ccosel(&self) -> CCOSEL_R {
        CCOSEL_R::new((self.bits >> 1) & 0x0f)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ccordy(&self) -> CCORDY_R {
        CCORDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cc0bsy(&self) -> CC0BSY_R {
        CC0BSY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ccoen(&mut self) -> CCOEN_W<CCOR_SPEC> {
        CCOEN_W::new(self, 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    #[must_use]
    pub fn ccosel(&mut self) -> CCOSEL_W<CCOR_SPEC> {
        CCOSEL_W::new(self, 1)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ccordy(&mut self) -> CCORDY_W<CCOR_SPEC> {
        CCORDY_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cc0bsy(&mut self) -> CC0BSY_W<CCOR_SPEC> {
        CC0BSY_W::new(self, 6)
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
#[doc = "Configurable clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCOR_SPEC;
impl crate::RegisterSpec for CCOR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccor::R`](R) reader structure"]
impl crate::Readable for CCOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccor::W`](W) writer structure"]
impl crate::Writable for CCOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCOR to value 0"]
impl crate::Resettable for CCOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
