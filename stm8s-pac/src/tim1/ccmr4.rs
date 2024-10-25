#[doc = "Register `CCMR4` reader"]
pub type R = crate::R<CCMR4_SPEC>;
#[doc = "Register `CCMR4` writer"]
pub type W = crate::W<CCMR4_SPEC>;
#[doc = "Field `CC4S` reader - "]
pub type CC4S_R = crate::FieldReader;
#[doc = "Field `CC4S` writer - "]
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC4FE` reader - "]
pub type OC4FE_R = crate::BitReader;
#[doc = "Field `OC4FE` writer - "]
pub type OC4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4PE` reader - "]
pub type OC4PE_R = crate::BitReader;
#[doc = "Field `OC4PE` writer - "]
pub type OC4PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M` reader - "]
pub type OC4M_R = crate::FieldReader;
#[doc = "Field `OC4M` writer - "]
pub type OC4M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC4CE` reader - "]
pub type OC4CE_R = crate::BitReader;
#[doc = "Field `OC4CE` writer - "]
pub type OC4CE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(self.bits & 3)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn oc4m(&self) -> OC4M_R {
        OC4M_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CC4S_W<CCMR4_SPEC> {
        CC4S_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn oc4fe(&mut self) -> OC4FE_W<CCMR4_SPEC> {
        OC4FE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oc4pe(&mut self) -> OC4PE_W<CCMR4_SPEC> {
        OC4PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn oc4m(&mut self) -> OC4M_W<CCMR4_SPEC> {
        OC4M_W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn oc4ce(&mut self) -> OC4CE_W<CCMR4_SPEC> {
        OC4CE_W::new(self, 7)
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
#[doc = "TIM1 capture/compare mode register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR4_SPEC;
impl crate::RegisterSpec for CCMR4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccmr4::R`](R) reader structure"]
impl crate::Readable for CCMR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr4::W`](W) writer structure"]
impl crate::Writable for CCMR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCMR4 to value 0"]
impl crate::Resettable for CCMR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
