use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.5 21 14 3" ></ path > < path d = "M20.5 21 10 3" ></ path > < path d = "M15.5 21 12 15l-3.5 6" ></ path > < path d = "M2 21h20" ></ path > < / > } } pub const LUCIDE_TENT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;