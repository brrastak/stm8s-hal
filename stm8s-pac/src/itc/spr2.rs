#[doc = "Register `SPR2` reader"]
pub type R = crate::R<SPR2_SPEC>;
#[doc = "Register `SPR2` writer"]
pub type W = crate::W<SPR2_SPEC>;
#[doc = "Field `VECT4SPR` reader - "]
pub type VECT4SPR_R = crate::FieldReader;
#[doc = "Field `VECT4SPR` writer - "]
pub type VECT4SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT5SPR` reader - "]
pub type VECT5SPR_R = crate::FieldReader;
#[doc = "Field `VECT5SPR` writer - "]
pub type VECT5SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT6SPR` reader - "]
pub type VECT6SPR_R = crate::FieldReader;
#[doc = "Field `VECT6SPR` writer - "]
pub type VECT6SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT7SPR` reader - "]
pub type VECT7SPR_R = crate::FieldReader;
#[doc = "Field `VECT7SPR` writer - "]
pub type VECT7SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vect4spr(&self) -> VECT4SPR_R {
        VECT4SPR_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vect5spr(&self) -> VECT5SPR_R {
        VECT5SPR_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn vect6spr(&self) -> VECT6SPR_R {
        VECT6SPR_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn vect7spr(&self) -> VECT7SPR_R {
        VECT7SPR_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn vect4spr(&mut self) -> VECT4SPR_W<SPR2_SPEC> {
        VECT4SPR_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn vect5spr(&mut self) -> VECT5SPR_W<SPR2_SPEC> {
        VECT5SPR_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn vect6spr(&mut self) -> VECT6SPR_W<SPR2_SPEC> {
        VECT6SPR_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn vect7spr(&mut self) -> VECT7SPR_W<SPR2_SPEC> {
        VECT7SPR_W::new(self, 6)
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
#[doc = "Interrupt software priority register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPR2_SPEC;
impl crate::RegisterSpec for SPR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spr2::R`](R) reader structure"]
impl crate::Readable for SPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spr2::W`](W) writer structure"]
impl crate::Writable for SPR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPR2 to value 0xff"]
impl crate::Resettable for SPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
