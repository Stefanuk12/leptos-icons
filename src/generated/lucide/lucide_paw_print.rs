use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "11" cy = "4" r = "2" ></ circle > < circle cy = "8" r = "2" cx = "18" ></ circle > < circle cy = "16" cx = "20" r = "2" ></ circle > < path d = "M9 10a5 5 0 0 1 5 5v3.5a3.5 3.5 0 0 1-6.84 1.045Q6.52 17.48 4.46 16.84A3.5 3.5 0 0 1 5.5 10Z" ></ path > < / > } } pub const LUCIDE_PAW_PRINT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;