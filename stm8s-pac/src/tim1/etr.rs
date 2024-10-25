#[doc = "Register `ETR` reader"]
pub type R = crate::R<ETR_SPEC>;
#[doc = "Register `ETR` writer"]
pub type W = crate::W<ETR_SPEC>;
#[doc = "Field `ETF` reader - "]
pub type ETF_R = crate::FieldReader;
#[doc = "Field `ETF` writer - "]
pub type ETF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETPS` reader - "]
pub type ETPS_R = crate::FieldReader;
#[doc = "Field `ETPS` writer - "]
pub type ETPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECE` reader - "]
pub type ECE_R = crate::BitReader;
#[doc = "Field `ECE` writer - "]
pub type ECE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETP` reader - "]
pub type ETP_R = crate::BitReader;
#[doc = "Field `ETP` writer - "]
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<ETR_SPEC> {
        ETF_W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<ETR_SPEC> {
        ETPS_W::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<ETR_SPEC> {
        ECE_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<ETR_SPEC> {
        ETP_W::new(self, 7)
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
#[doc = "TIM1 external trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETR_SPEC;
impl crate::RegisterSpec for ETR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`etr::R`](R) reader structure"]
impl crate::Readable for ETR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etr::W`](W) writer structure"]
impl crate::Writable for ETR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETR to value 0"]
impl crate::Resettable for ETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
