use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "11" r = "8" cx = "11" ></ circle > < path d = "m21 21-4.3-4.3" ></ path > < path d = "M11 11a2 2 0 0 0 4 0 4 4 0 0 0-8 0 6 6 0 0 0 12 0" ></ path > < / > } } pub const LUCIDE_LOLLIPOP : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;