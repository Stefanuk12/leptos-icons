use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 15H6a4 4 0 0 0-4 4v2" ></ path > < path d = "m21.7 16.4-.9-.3" ></ path > < path d = "m15.2 13.9-.9-.3" ></ path > < path d = "m16.6 18.7.3-.9" ></ path > < path d = "m19.1 12.2.3-.9" ></ path > < path d = "m19.6 18.7-.4-1" ></ path > < path d = "m16.8 12.3-.4-1" ></ path > < path d = "m14.3 16.6 1-.4" ></ path > < path d = "m20.7 13.8 1-.4" ></ path > < / > } } pub const LucideUserCog : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;