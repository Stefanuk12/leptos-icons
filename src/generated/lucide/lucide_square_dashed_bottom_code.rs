use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 9.5 8 12l2 2.5" ></ path > < path d = "M14 21h1" ></ path > < path d = "m14 9.5 2 2.5-2 2.5" ></ path > < path d = "M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2" ></ path > < path d = "M9 21h1" ></ path > < / > } } pub const LUCIDE_SQUARE_DASHED_BOTTOM_CODE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;