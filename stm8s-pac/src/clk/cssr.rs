#[doc = "Register `CSSR` reader"]
pub type R = crate::R<CSSR_SPEC>;
#[doc = "Register `CSSR` writer"]
pub type W = crate::W<CSSR_SPEC>;
#[doc = "Field `CSSEN` reader - "]
pub type CSSEN_R = crate::BitReader;
#[doc = "Field `CSSEN` writer - "]
pub type CSSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX` reader - "]
pub type AUX_R = crate::BitReader;
#[doc = "Field `AUX` writer - "]
pub type AUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSDIE` reader - "]
pub type CSSDIE_R = crate::BitReader;
#[doc = "Field `CSSDIE` writer - "]
pub type CSSDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSD` reader - "]
pub type CSSD_R = crate::BitReader;
#[doc = "Field `CSSD` writer - "]
pub type CSSD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cssen(&self) -> CSSEN_R {
        CSSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn aux(&self) -> AUX_R {
        AUX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cssdie(&self) -> CSSDIE_R {
        CSSDIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cssd(&self) -> CSSD_R {
        CSSD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cssen(&mut self) -> CSSEN_W<CSSR_SPEC> {
        CSSEN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn aux(&mut self) -> AUX_W<CSSR_SPEC> {
        AUX_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cssdie(&mut self) -> CSSDIE_W<CSSR_SPEC> {
        CSSDIE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cssd(&mut self) -> CSSD_W<CSSR_SPEC> {
        CSSD_W::new(self, 3)
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
#[doc = "Clock security system register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSSR_SPEC;
impl crate::RegisterSpec for CSSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cssr::R`](R) reader structure"]
impl crate::Readable for CSSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cssr::W`](W) writer structure"]
impl crate::Writable for CSSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSSR to value 0"]
impl crate::Resettable for CSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
