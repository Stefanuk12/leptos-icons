use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M19 15v-2a2 2 0 1 0-4 0v2" ></ path > < path d = "M9 17H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v3.5" ></ path > < rect x = "13" y = "15" width = "8" height = "5" rx = "1" ></ rect > < / > } } pub const LUCIDE_MESSAGE_SQUARE_LOCK : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24")] } ;