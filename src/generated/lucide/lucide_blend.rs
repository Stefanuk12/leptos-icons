use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "9" r = "7" cy = "9" ></ circle > < circle cx = "15" r = "7" cy = "15" ></ circle > < / > } } pub const LUCIDE_BLEND : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;