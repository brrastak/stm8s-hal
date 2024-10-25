#[doc = "Register `SPR7` reader"]
pub type R = crate::R<SPR7_SPEC>;
#[doc = "Register `SPR7` writer"]
pub type W = crate::W<SPR7_SPEC>;
#[doc = "Field `VECT24SPR` reader - "]
pub type VECT24SPR_R = crate::FieldReader;
#[doc = "Field `VECT24SPR` writer - "]
pub type VECT24SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT25SPR` reader - "]
pub type VECT25SPR_R = crate::FieldReader;
#[doc = "Field `VECT25SPR` writer - "]
pub type VECT25SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT26SPR` reader - "]
pub type VECT26SPR_R = crate::FieldReader;
#[doc = "Field `VECT26SPR` writer - "]
pub type VECT26SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT27SPR` reader - "]
pub type VECT27SPR_R = crate::FieldReader;
#[doc = "Field `VECT27SPR` writer - "]
pub type VECT27SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vect24spr(&self) -> VECT24SPR_R {
        VECT24SPR_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vect25spr(&self) -> VECT25SPR_R {
        VECT25SPR_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn vect26spr(&self) -> VECT26SPR_R {
        VECT26SPR_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn vect27spr(&self) -> VECT27SPR_R {
        VECT27SPR_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn vect24spr(&mut self) -> VECT24SPR_W<SPR7_SPEC> {
        VECT24SPR_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn vect25spr(&mut self) -> VECT25SPR_W<SPR7_SPEC> {
        VECT25SPR_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn vect26spr(&mut self) -> VECT26SPR_W<SPR7_SPEC> {
        VECT26SPR_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn vect27spr(&mut self) -> VECT27SPR_W<SPR7_SPEC> {
        VECT27SPR_W::new(self, 6)
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
#[doc = "Interrupt software priority register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPR7_SPEC;
impl crate::RegisterSpec for SPR7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spr7::R`](R) reader structure"]
impl crate::Readable for SPR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spr7::W`](W) writer structure"]
impl crate::Writable for SPR7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPR7 to value 0xff"]
impl crate::Resettable for SPR7_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
