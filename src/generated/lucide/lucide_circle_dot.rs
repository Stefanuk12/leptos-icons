use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < circle cx = "12" r = "1" cy = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE_DOT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;