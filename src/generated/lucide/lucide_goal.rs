use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 13V2l8 4-8 4" ></ path > < path d = "M20.561 10.222a9 9 0 1 1-12.55-5.29" ></ path > < path d = "M8.002 9.997a5 5 0 1 0 8.9 2.02" ></ path > < / > } } pub const LUCIDE_GOAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24")] } ;