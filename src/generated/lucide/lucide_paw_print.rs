use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "11" cy = "4" r = "2" ></ circle > < circle cx = "18" cy = "8" r = "2" ></ circle > < circle cx = "20" cy = "16" r = "2" ></ circle > < path d = "M9 10a5 5 0 0 1 5 5v3.5a3.5 3.5 0 0 1-6.84 1.045Q6.52 17.48 4.46 16.84A3.5 3.5 0 0 1 5.5 10Z" ></ path > < / > } } pub const LUCIDE_PAW_PRINT : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;