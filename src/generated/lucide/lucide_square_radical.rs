use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 12h2l2 5 2-10h4" ></ path > < rect height = "18" rx = "2" y = "3" width = "18" x = "3" ></ rect > < / > } } pub const LUCIDE_SQUARE_RADICAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;