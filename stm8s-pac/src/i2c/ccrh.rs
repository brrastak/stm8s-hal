#[doc = "Register `CCRH` reader"]
pub type R = crate::R<CCRH_SPEC>;
#[doc = "Register `CCRH` writer"]
pub type W = crate::W<CCRH_SPEC>;
#[doc = "Field `CCR` reader - "]
pub type CCR_R = crate::FieldReader;
#[doc = "Field `CCR` writer - "]
pub type CCR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DUTY` reader - "]
pub type DUTY_R = crate::BitReader;
#[doc = "Field `DUTY` writer - "]
pub type DUTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F_S` reader - "]
pub type F_S_R = crate::BitReader;
#[doc = "Field `F_S` writer - "]
pub type F_S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn f_s(&self) -> F_S_R {
        F_S_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn ccr(&mut self) -> CCR_W<CCRH_SPEC> {
        CCR_W::new(self, 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<CCRH_SPEC> {
        DUTY_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn f_s(&mut self) -> F_S_W<CCRH_SPEC> {
        F_S_W::new(self, 7)
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
#[doc = "I2C Clock control register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRH_SPEC;
impl crate::RegisterSpec for CCRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccrh::R`](R) reader structure"]
impl crate::Readable for CCRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccrh::W`](W) writer structure"]
impl crate::Writable for CCRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCRH to value 0"]
impl crate::Resettable for CCRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
