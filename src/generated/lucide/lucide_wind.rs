use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17.7 7.7a2.5 2.5 0 1 1 1.8 4.3H2" ></ path > < path d = "M9.6 4.6A2 2 0 1 1 11 8H2" ></ path > < path d = "M12.6 19.4A2 2 0 1 0 14 16H2" ></ path > < / > } } pub const LUCIDE_WIND : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;