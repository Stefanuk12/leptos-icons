use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" ></ path > < path d = "M13 8H7" ></ path > < path d = "M17 12H7" ></ path > < / > } } pub const LUCIDE_MESSAGE_SQUARE_TEXT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;