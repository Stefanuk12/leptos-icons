use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 3v12" ></ path > < path d = "M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" ></ path > < path d = "M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" ></ path > < path d = "M15 6a9 9 0 0 0-9 9" ></ path > < path d = "M18 15v6" ></ path > < path d = "M21 18h-6" ></ path > < / > } } pub const LUCIDE_GIT_BRANCH_PLUS : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;