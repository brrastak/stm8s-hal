#[doc = "Register `RST_SR` reader"]
pub type R = crate::R<RST_SR_SPEC>;
#[doc = "Register `RST_SR` writer"]
pub type W = crate::W<RST_SR_SPEC>;
#[doc = "Field `WWDGF` reader - "]
pub type WWDGF_R = crate::BitReader;
#[doc = "Field `WWDGF` writer - "]
pub type WWDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDGF` reader - "]
pub type IWDGF_R = crate::BitReader;
#[doc = "Field `IWDGF` writer - "]
pub type IWDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILLOPF` reader - "]
pub type ILLOPF_R = crate::BitReader;
#[doc = "Field `ILLOPF` writer - "]
pub type ILLOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIMF` reader - "]
pub type SWIMF_R = crate::BitReader;
#[doc = "Field `SWIMF` writer - "]
pub type SWIMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMCF` reader - "]
pub type EMCF_R = crate::BitReader;
#[doc = "Field `EMCF` writer - "]
pub type EMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wwdgf(&self) -> WWDGF_R {
        WWDGF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn iwdgf(&self) -> IWDGF_R {
        IWDGF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn illopf(&self) -> ILLOPF_R {
        ILLOPF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swimf(&self) -> SWIMF_R {
        SWIMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn emcf(&self) -> EMCF_R {
        EMCF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgf(&mut self) -> WWDGF_W<RST_SR_SPEC> {
        WWDGF_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iwdgf(&mut self) -> IWDGF_W<RST_SR_SPEC> {
        IWDGF_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn illopf(&mut self) -> ILLOPF_W<RST_SR_SPEC> {
        ILLOPF_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn swimf(&mut self) -> SWIMF_W<RST_SR_SPEC> {
        SWIMF_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn emcf(&mut self) -> EMCF_W<RST_SR_SPEC> {
        EMCF_W::new(self, 4)
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
#[doc = "Reset status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST_SR_SPEC;
impl crate::RegisterSpec for RST_SR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rst_sr::R`](R) reader structure"]
impl crate::Readable for RST_SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst_sr::W`](W) writer structure"]
impl crate::Writable for RST_SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RST_SR to value 0"]
impl crate::Resettable for RST_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
