use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 13V7" ></ path > < path d = "M18 2h1a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20" ></ path > < path d = "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2" ></ path > < path d = "m9 10 3-3 3 3" ></ path > < path d = "m9 5 3-3 3 3" ></ path > < / > } } pub const LUCIDE_BOOK_UP_2 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;