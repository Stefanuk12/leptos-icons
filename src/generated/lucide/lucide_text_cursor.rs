use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 22h-1a4 4 0 0 1-4-4V6a4 4 0 0 1 4-4h1" ></ path > < path d = "M7 22h1a4 4 0 0 0 4-4v-1" ></ path > < path d = "M7 2h1a4 4 0 0 1 4 4v1" ></ path > < / > } } pub const LucideTextCursor : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;