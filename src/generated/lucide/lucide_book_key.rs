use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H14" ></ path > < path d = "M20 8v14H6.5a2.5 2.5 0 0 1 0-5H20" ></ path > < circle cy = "8" cx = "14" r = "2" ></ circle > < path d = "m20 2-4.5 4.5" ></ path > < path d = "m19 3 1 1" ></ path > < / > } } pub const LUCIDE_BOOK_KEY : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;