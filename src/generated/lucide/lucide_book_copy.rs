use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 16V4a2 2 0 0 1 2-2h11" ></ path > < path d = "M5 14H4a2 2 0 1 0 0 4h1" ></ path > < path d = "M22 18H11a2 2 0 1 0 0 4h11V6H11a2 2 0 0 0-2 2v12" ></ path > < / > } } pub const LucideBookCopy : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;