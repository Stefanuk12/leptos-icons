use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" ></ path > < path d = "m15 11-6 6" ></ path > < path d = "m9 11 6 6" ></ path > < / > } } pub const LucideClipboardX : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;