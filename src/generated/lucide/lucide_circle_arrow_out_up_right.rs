use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 12A10 10 0 1 1 12 2" ></ path > < path d = "M22 2 12 12" ></ path > < path d = "M16 2h6v6" ></ path > < / > } } pub const LUCIDE_CIRCLE_ARROW_OUT_UP_RIGHT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;