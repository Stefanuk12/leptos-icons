use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21.801 10A10 10 0 1 1 17 3.335" ></ path > < path d = "m9 11 3 3L22 4" ></ path > < / > } } pub const LUCIDE_CIRCLE_CHECK_BIG : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;