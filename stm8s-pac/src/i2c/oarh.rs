#[doc = "Register `OARH` reader"]
pub type R = crate::R<OARH_SPEC>;
#[doc = "Register `OARH` writer"]
pub type W = crate::W<OARH_SPEC>;
#[doc = "Field `ADD` reader - "]
pub type ADD_R = crate::FieldReader;
#[doc = "Field `ADD` writer - "]
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDCONF` reader - "]
pub type ADDCONF_R = crate::BitReader;
#[doc = "Field `ADDCONF` writer - "]
pub type ADDCONF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDMODE` reader - "]
pub type ADDMODE_R = crate::BitReader;
#[doc = "Field `ADDMODE` writer - "]
pub type ADDMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn addconf(&self) -> ADDCONF_R {
        ADDCONF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn addmode(&self) -> ADDMODE_R {
        ADDMODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<OARH_SPEC> {
        ADD_W::new(self, 1)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn addconf(&mut self) -> ADDCONF_W<OARH_SPEC> {
        ADDCONF_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn addmode(&mut self) -> ADDMODE_W<OARH_SPEC> {
        ADDMODE_W::new(self, 7)
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
#[doc = "I2C Own address register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oarh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oarh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OARH_SPEC;
impl crate::RegisterSpec for OARH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oarh::R`](R) reader structure"]
impl crate::Readable for OARH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oarh::W`](W) writer structure"]
impl crate::Writable for OARH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OARH to value 0"]
impl crate::Resettable for OARH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
