use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "11" cy = "4" r = "2" ></ circle > < circle cy = "8" r = "2" cx = "18" ></ circle > < circle cx = "20" r = "2" cy = "16" ></ circle > < path d = "M9 10a5 5 0 0 1 5 5v3.5a3.5 3.5 0 0 1-6.84 1.045Q6.52 17.48 4.46 16.84A3.5 3.5 0 0 1 5.5 10Z" ></ path > < / > } } pub const LUCIDE_PAW_PRINT : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;