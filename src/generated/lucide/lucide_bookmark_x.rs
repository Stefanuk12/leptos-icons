use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2Z" ></ path > < path d = "m14.5 7.5-5 5" ></ path > < path d = "m9.5 7.5 5 5" ></ path > < / > } } pub const LUCIDE_BOOKMARK_X : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;