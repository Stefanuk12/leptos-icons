use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z" ></ path > < line x1 = "15" y1 = "10" y2 = "10" x2 = "9" ></ line > < / > } } pub const LUCIDE_BOOKMARK_MINUS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;