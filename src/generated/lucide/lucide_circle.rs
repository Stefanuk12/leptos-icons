use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;