use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" ></ path > < path d = "M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" ></ path > < / > } } pub const LucideBookOpen : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round")] } ;