use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m8 2 1.88 1.88" ></ path > < path d = "M14.12 3.88 16 2" ></ path > < path d = "M9 7.13v-1a3.003 3.003 0 1 1 6 0v1" ></ path > < path d = "M18 11a4 4 0 0 0-4-4h-4a4 4 0 0 0-4 4v3a6.1 6.1 0 0 0 2 4.5" ></ path > < path d = "M6.53 9C4.6 8.8 3 7.1 3 5" ></ path > < path d = "M6 13H2" ></ path > < path d = "M3 21c0-2.1 1.7-3.9 3.8-4" ></ path > < path d = "M20.97 5c0 2.1-1.6 3.8-3.5 4" ></ path > < path d = "m12 12 8 5-8 5Z" ></ path > < / > } } pub const LucideBugPlay : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;