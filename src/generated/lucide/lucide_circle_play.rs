use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < polygon points = "10 8 16 12 10 16 10 8" ></ polygon > < / > } } pub const LUCIDE_CIRCLE_PLAY : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none")] } ;