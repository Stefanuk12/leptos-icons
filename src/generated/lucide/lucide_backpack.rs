use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 10a4 4 0 0 1 4-4h8a4 4 0 0 1 4 4v10a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2z" ></ path > < path d = "M8 10h8" ></ path > < path d = "M8 18h8" ></ path > < path d = "M8 22v-6a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v6" ></ path > < path d = "M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2" ></ path > < / > } } pub const LUCIDE_BACKPACK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;