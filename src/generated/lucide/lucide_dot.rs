use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "1" cx = "12.1" cy = "12.1" ></ circle > < / > } } pub const LUCIDE_DOT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;