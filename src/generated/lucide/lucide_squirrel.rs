use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15.236 22a3 3 0 0 0-2.2-5" ></ path > < path d = "M16 20a3 3 0 0 1 3-3h1a2 2 0 0 0 2-2v-2a4 4 0 0 0-4-4V4" ></ path > < path d = "M18 13h.01" ></ path > < path d = "M18 6a4 4 0 0 0-4 4 7 7 0 0 0-7 7c0-5 4-5 4-10.5a4.5 4.5 0 1 0-9 0 2.5 2.5 0 0 0 5 0C7 10 3 11 3 17c0 2.8 2.2 5 5 5h10" ></ path > < / > } } pub const LUCIDE_SQUIRREL : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;