use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M32 64c0-17.7 14.3-32 32-32l512 0c17.7 0 32 14.3 32 32s-14.3 32-32 32l-40 0 0 64-8 0c-61.9 0-112 50.1-112 112l0 24.6c-9.9 5.8-18.2 14.1-23.8 24.1c-17.6-20-43.4-32.7-72.2-32.7c-53 0-96 43-96 96l0 64c0 17.7-14.3 32-32 32l-32 0c-17.7 0-32-14.3-32-32l0-64c0-53-43-96-96-96l0-128 72 0 0-64L64 96C46.3 96 32 81.7 32 64zM408 96l0 64 80 0 0-64-80 0zm-48 64l0-64-80 0 0 64 80 0zM152 96l0 64 80 0 0-64-80 0zM528 240c-17.7 0-32 14.3-32 32l0 48 64 0 0-48c0-17.7-14.3-32-32-32zm-80 32c0-44.2 35.8-80 80-80s80 35.8 80 80l0 48c17.7 0 32 14.3 32 32l0 128c0 17.7-14.3 32-32 32l-160 0c-17.7 0-32-14.3-32-32l0-128c0-17.7 14.3-32 32-32l0-48z" ></ path > < / > } } pub const FONT_AWESOME_SOLID_BRIDGE_LOCK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 640 512")] } ;