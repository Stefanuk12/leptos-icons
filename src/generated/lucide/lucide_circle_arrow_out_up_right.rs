use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 12A10 10 0 1 1 12 2" ></ path > < path d = "M22 2 12 12" ></ path > < path d = "M16 2h6v6" ></ path > < / > } } pub const LUCIDE_CIRCLE_ARROW_OUT_UP_RIGHT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24")] } ;