use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "13 19 22 12 13 5 13 19" ></ polygon > < polygon points = "2 19 11 12 2 5 2 19" ></ polygon > < / > } } pub const LUCIDE_FAST_FORWARD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;